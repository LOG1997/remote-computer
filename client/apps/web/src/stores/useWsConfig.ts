import { create } from 'zustand';
import { persist } from 'zustand/middleware';
// 1. 定义 Store 类型（TS 可选，但推荐）
interface WsConfigStore {
    wsConfig: {
        host: string;
        port: number;
        path: string;
        securityKey?: string;
        // username?: string;
        // password?: string;
        // topicName: string;
    } | null;
    setConfig: (info: WsConfigStore['wsConfig']) => void;
    clearConfig: () => void;
}

// 2. 创建持久化 Store
export const useWsConfig = create<WsConfigStore>()(
    persist(
        (set) => ({
            // 初始状态
            wsConfig: null,
            // 修改数据的方法
            setConfig: (data) => set({ wsConfig: data }),
            clearConfig: () => set({ wsConfig: null }),
        }),
        {
            // 🔥 关键配置：持久化名称（唯一标识 storage key）
            name: 'ws-config-storage',

            // 👇 可选：自定义存储方式（默认 localStorage）
            // storage: createJSONStorage(() => sessionStorage),
        }
    )
);