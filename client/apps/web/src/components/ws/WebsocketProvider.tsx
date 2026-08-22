// websocket/WebSocketProvider.tsx
import { useWsConfig } from '@/stores';
import React, {
    createContext,
    useContext,
    useEffect,
    useRef,
    useState,
    useCallback,
    type ReactNode,
} from 'react';

// ---------- 类型定义 ----------
interface WebSocketContextValue {
    /** 发送消息（自动 JSON 序列化） */
    sendMessage: (data: unknown) => void;
    /** 订阅消息，按 type 过滤，返回取消订阅函数 */
    subscribe: (callback: (data: unknown) => void, type?: string) => () => void;
    /** 当前连接状态（WebSocket.readyState 的值） */
    readyState: number;
}

const WebSocketContext = createContext<WebSocketContextValue | null>(null);

// ---------- 自定义 Hook（供组件使用） ----------
export const useWebSocket = () => {
    const context = useContext(WebSocketContext);
    if (!context) {
        throw new Error('useWebSocket 必须在 WebSocketProvider 内部使用');
    }
    return context;
};

// ---------- Provider 组件 ----------
interface WebSocketProviderProps {
    children: ReactNode;
    /** 重连间隔（毫秒），默认 3000 */
    reconnectInterval?: number;
}

export const WebSocketProvider: React.FC<WebSocketProviderProps> = ({
    children,
    reconnectInterval = 10000,
}) => {

    const [readyState, setReadyState] = useState<number>(WebSocket.CONNECTING);
    const wsRef = useRef<WebSocket | null>(null);
    const reconnectTimerRef = useRef<number | null>(null);
    // 存储订阅者：{ type: Set<callback> }
    const listenersRef = useRef<Map<string, Set<(data: unknown) => void>>>(new Map());

    const wsConfig = useWsConfig((state) => state.wsConfig);
    const buildUrl = useCallback(() => {
        // if (!wsConfig) return ''; // 配置未加载
        const { host, port, path, securityKey } = wsConfig || {};
        const finalHost = host || window.location.hostname || '127.0.0.1';
        const finalPort = port || '52011';
        const normalizedPath = path?.startsWith('/') ? path : `/user`;
        const query = securityKey ? `?token=${encodeURIComponent(securityKey)}` : `?token=${1212121}`;
        const result_url = `ws://${finalHost}:${finalPort}${normalizedPath}${query}`
        return result_url;
    }, [wsConfig]);
    // ---------- 建立连接 ----------
    const connect = useCallback(() => {
        const currentUrl = buildUrl(); // 获取最新的 URL
        if (!currentUrl) {
            console.warn('⚠️ WebSocket URL 为空，放弃连接');
            if (wsRef.current) {
                wsRef.current.close();
                wsRef.current = null;
            }
            setReadyState(WebSocket.CLOSED);
            return;
        }
        // 清除旧连接
        if (wsRef.current) {
            wsRef.current.close();
            wsRef.current = null;
        }

        const ws = new WebSocket(currentUrl);
        wsRef.current = ws;

        ws.onopen = () => {
            setReadyState(WebSocket.OPEN);
            console.log('✅ WebSocket 已连接');
            // 清除重连定时器
            if (reconnectTimerRef.current) {
                clearTimeout(reconnectTimerRef.current);
                reconnectTimerRef.current = null;
            }
        };

        ws.onclose = (event) => {
            setReadyState(WebSocket.CLOSED);
            console.warn('⚠️ WebSocket 断开，尝试重连...', event);
            // 自动重连（未正常关闭时）
            if (!event.wasClean && reconnectInterval > 0) {
                reconnectTimerRef.current = window.setTimeout(() => {
                    connect();
                }, reconnectInterval);
            }
        };

        ws.onerror = (error) => {
            console.error('❌ WebSocket 错误', currentUrl, error);
        };

        ws.onmessage = (event) => {
            let rawData: unknown;
            try {
                rawData = JSON.parse(event.data);
            } catch {
                rawData = event.data; // 非 JSON 时直接透传
            }

            // 约定消息格式：{ type?: string, data: any }
            // 若没有 type，则视为 type = 'default'
            const type = (rawData as any)?.topic || 'default';
            const payload = (rawData as any)?.data ?? rawData;

            const listeners = listenersRef.current;
            // 触发指定类型的回调
            const callbacks = listeners.get(type);
            if (callbacks) {
                callbacks.forEach((cb) => cb(payload));
            }
            // 同时触发通配 '*' 的订阅（如果存在）
            const allCallbacks = listeners.get('*');
            if (allCallbacks) {
                allCallbacks.forEach((cb) => cb(payload));
            }
        };
    }, [buildUrl, reconnectInterval]);

    // ---------- 生命周期：挂载时连接，卸载时断开 ----------
    useEffect(() => {
        const currentUrl = buildUrl();
        if (!currentUrl) {
            console.warn('⚠️ WebSocket 未配置 URL，请检查');
            return
        }
        connect();
        return () => {
            if (reconnectTimerRef.current) {
                clearTimeout(reconnectTimerRef.current);
                reconnectTimerRef.current = null;
            }
            if (wsRef.current) {
                wsRef.current.close();
                wsRef.current = null;
            }
        };
    }, [connect, buildUrl]);

    // ---------- 发送消息 ----------
    const sendMessage = useCallback((data: unknown) => {

        if (wsRef.current?.readyState === WebSocket.OPEN) {
            wsRef.current.send(JSON.stringify(data));
        } else {
            console.warn('⚠️ WebSocket 未打开，消息未发送');
        }
    }, []);

    // ---------- 订阅 / 取消订阅 ----------
    const subscribe = useCallback((callback: (data: unknown) => void, type = '*') => {
        const listeners = listenersRef.current;
        if (!listeners.has(type)) {
            listeners.set(type, new Set());
        }
        listeners.get(type)!.add(callback);

        // 返回取消订阅函数
        return () => {
            const callbacks = listeners.get(type);
            if (callbacks) {
                callbacks.delete(callback);
                if (callbacks.size === 0) {
                    listeners.delete(type);
                }
            }
        };
    }, []);

    const contextValue: WebSocketContextValue = {
        sendMessage,
        subscribe,
        readyState,
    };

    return (
        <WebSocketContext.Provider value={contextValue}>
            {children}
        </WebSocketContext.Provider>
    );
};
