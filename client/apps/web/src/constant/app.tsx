import { OrayIcon, WechatIcon, BilibiliIcon, ClashIcon, SteamIcon } from '@/components/icons'

export const AppLaunchList = [
    {
        name: '向日葵',
        value: 'oray',
        description: '向日葵远程软件',
        icon: <OrayIcon className="w-10 h-10" />,
    },
    {
        name: '微信',
        value: 'wx',
        description: '微信聊天软件',
        icon: <WechatIcon className="w-10 h-10 text-green-400" />,
    },
    {
        name: 'Bilibili',
        value: 'bl',
        description: 'bilibili视频软件',
        icon: <BilibiliIcon className="w-10 h-10 text-pink-400" />,
    },
    {
        name: 'ClashVerge',
        value: 'clash',
        description: 'clash代理软件',
        icon: <ClashIcon className="w-10 h-10" />,
    },
    {
        name: 'Steam',
        value: 'steam',
        description: 'steam游戏软件',
        icon: <SteamIcon className="w-10 h-10" />,
    }
]