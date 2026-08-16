import { create } from 'zustand';
import { persist } from 'zustand/middleware';

// 1. 定义 Store 类型（TS 可选，但推荐）
interface PlatformStore {
    platformInfo: {
        current: any;
    } | null;
    setConfig: (info: PlatformStore['platformInfo']) => void;
    clearConfig: () => void;
}

// 2. 创建持久化 Store
export const usePlatform = create<PlatformStore>()(
    persist(
        (set) => ({
            // 初始状态
            platformInfo: null,

            // 修改数据的方法
            setConfig: (data) => set({ platformInfo: data }),
            clearConfig: () => set({ platformInfo: null }),
        }),
        {
            name: 'platform-storage',

            // 👇 可选：自定义存储方式（默认 localStorage）
            // storage: createJSONStorage(() => sessionStorage),
        }
    )
);