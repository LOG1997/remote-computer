import { create } from 'zustand';
import { persist } from 'zustand/middleware';

// 1. 定义 Store 类型（TS 可选，但推荐）
interface ConfigurationStore {
    config: {
        host: string;
        port: string;
    } | null;
    setConfig: (info: ConfigurationStore['config']) => void;
    clearConfig: () => void;
}

// 2. 创建持久化 Store
export const useConfigurationStore = create<ConfigurationStore>()(
    persist(
        (set) => ({
            // 初始状态
            config: null,

            // 修改数据的方法
            setConfig: (data) => set({ config: data }),
            clearConfig: () => set({ config: null }),
        }),
        {
            // 🔥 关键配置：持久化名称（唯一标识 storage key）
            name: 'config-storage',

            // 👇 可选：自定义存储方式（默认 localStorage）
            // storage: createJSONStorage(() => sessionStorage),
        }
    )
);