import { BilibiliIcon, TencentVideo } from '@/components/icons'
export const menus = [
    {
        name: "bilibili",
        icon: <BilibiliIcon className='size-10 text-pink-400' />,
        url: "https://space.bilibili.com/",
        route: "bilibili",
    },
    {
        name: "腾讯视频",
        icon: <TencentVideo className='size-10' />,
        url: "https://v.qq.com",
        route: "tencent-video",
    }
]