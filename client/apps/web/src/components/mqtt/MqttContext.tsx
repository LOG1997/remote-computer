/* eslint-disable react-refresh/only-export-components */
import { createContext, useContext, useEffect, useState, useRef, useCallback } from 'react';
import type { ReactNode } from 'react';
import mqtt from 'mqtt';
import type { MqttClient } from 'mqtt'
import { useMqttConfig } from '@/stores'

// -------------- 类型定义 --------------
export interface MqttMessage {
    topic: string;
    payload: string;
    time: string;
}

// MQTT 配置类型（和你的 store 结构对应）
export interface MqttConfig {
    address: string;
    port: number;
    path: string;
    username?: string;
    password?: string;
    topicName: string;
}
export type MqttMessageHandler = (msg: MqttMessage) => void;
interface MqttContextType {
    client: MqttClient | null;
    messages: MqttMessage[];
    isConnected: boolean;
    subscribe: (topic: string) => void;
    publish: (topic: string, message: any) => void;
    onMessage: (handler: MqttMessageHandler) => () => void;
}

const MqttContext = createContext<MqttContextType | undefined>(undefined);

export function MqttProvider({ children }: { children: ReactNode }) {
    const [client, setClient] = useState<MqttClient | null>(null);
    const [messages, setMessages] = useState<MqttMessage[]>([]);
    const [isConnected, setIsConnected] = useState(false);

    const handlersRef = useRef<Set<MqttMessageHandler>>(new Set());
    const configData = useMqttConfig((state) => state.mqttConfig)
    const { protocol } = window.location
    // 生成连接地址
    const getMqttUrl = (config: MqttConfig) => {
        const ws_protocol = protocol === 'https:' ? 'wss' : 'ws'
        return `${ws_protocol}://${config.address}:${config.port}${'/mqtt'}`;
    };

    // 连接 MQTT
    useEffect(() => {
        // 如果配置不完整，不连接
        if (!configData?.address || !configData?.port) return;

        const url = getMqttUrl(configData);
        console.log('正在连接 MQTT：', url);

        // 客户端配置
        const options = {
            clientId: 'react-app-' + Math.random().toString(16).slice(2, 10),
            username: configData.username,
            password: configData.password,
            clean: true,
            connectTimeout: 5000,
        };



        const mqttClient = mqtt.connect(url, options);

        // 连接成功
        mqttClient.on('connect', () => {
            console.log('✅ MQTT 连接成功');
            setIsConnected(true);
            setClient(mqttClient);
        });

        // 接收消息
        mqttClient.on('message', (topic, payload) => {
            console.log('Received message:', topic, payload.toString());
            const msg: MqttMessage = {
                topic,
                payload: payload.toString(),
                time: new Date().toLocaleString(),
            };
            setMessages(prev => [...prev, msg]);
            handlersRef.current.forEach(handler => {
                try {
                    handler(msg);
                } catch (error) {
                    console.error('MQTT 消息回调执行错误:', error);
                }
            });
        });

        // 错误/断开
        mqttClient.on('error', (err) => console.error('MQTT 错误：', err));
        mqttClient.on('close', () => setIsConnected(false));

        // 断开旧连接
        return () => {
            if (mqttClient) mqttClient.end();
        };
    }, [configData]); // 配置变化 → 自动重连

    // 订阅
    const subscribe = (topic: string) => {
        client?.subscribe(topic, (err) => {
            if (!err) console.log('已订阅：', topic);
        });
    };

    // 发布
    const publish = (topic: string, message: any) => {
        let messageResult = message;
        if (typeof message !== 'string') {
            messageResult = JSON.stringify(message);
        }
        client?.publish(topic, messageResult);
    };
    const onMessage = useCallback((handler: MqttMessageHandler) => {
        console.log('MQTT 添加消息监听器：', handler);
        handlersRef.current.add(handler);
        // 返回注销函数
        return () => {
            handlersRef.current.delete(handler);
        };
    }, []);
    return (
        <MqttContext.Provider value={{
            client,
            messages,
            isConnected,
            subscribe,
            publish,
            onMessage
        }}>
            {children}
        </MqttContext.Provider>
    );
}

// 自定义 Hook
export function useMqtt() {
    const ctx = useContext(MqttContext);
    if (!ctx) throw new Error('useMqtt 必须在 MqttProvider 内使用');
    return ctx;
}