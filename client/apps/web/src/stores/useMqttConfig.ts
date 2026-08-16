import { create } from 'zustand';
import { persist } from 'zustand/middleware';

// 1. 定义 Store 类型（TS 可选，但推荐）
interface MqttConfigStore {
    mqttConfig: {
        address: string;
        port: number;
        path: string;
        username?: string;
        password?: string;
        topicName: string;
    } | null;
    setConfig: (info: MqttConfigStore['mqttConfig']) => void;
    clearConfig: () => void;
}

// 2. 创建持久化 Store
export const useMqttConfig = create<MqttConfigStore>()(
    persist(
        (set) => ({
            // 初始状态
            mqttConfig: null,

            // 修改数据的方法
            setConfig: (data) => set({ mqttConfig: data }),
            clearConfig: () => set({ mqttConfig: null }),
        }),
        {
            // 🔥 关键配置：持久化名称（唯一标识 storage key）
            name: 'config-storage',

            // 👇 可选：自定义存储方式（默认 localStorage）
            // storage: createJSONStorage(() => sessionStorage),
        }
    )
);