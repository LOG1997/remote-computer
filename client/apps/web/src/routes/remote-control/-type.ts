export const RemoteControlTypeList = {
    UP: 'up', // 按键上
    DOWN: 'down', // 按键下 
    LEFT: 'left', // 按键左
    RIGHT: 'right', // 按键右
    OK: 'ok', // 确定
    BACK: 'back', // 返回 || 取消
    HOME: 'home', // 主菜单
    VOLUME_UP: 'volumeUp', // 音量加
    VOLUME_DOWN: 'volumeDown', // 音量减
    NEXT: 'next', // 下一个
    PREV: 'prev', // 上一个
} as const;
export type RemoteControlType = typeof RemoteControlTypeList[keyof typeof RemoteControlTypeList];

export interface RemoteControl {
    type: RemoteControlType;
    time: string;
}