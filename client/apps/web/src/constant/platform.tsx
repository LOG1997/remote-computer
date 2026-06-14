import { BilibiliIcon } from '@/components/icons/Bilibili'
import { YoutubeIcon } from '@/components/icons/Youtube'
export const PlatformList = [{
    name: 'bilibili',
    value: 'bilibili',
    path: '/bilibili',
    url: 'https://www.bilibili.com/',
    description: '哔哩哔哩',
    icon: <BilibiliIcon className="w-10 h-10 text-pink-400" />,
}, {
    name: 'youtube',
    value: 'youtube',
    path: '/youtube',
    url: 'https://www.youtube.com/',
    description: 'YouTube',
    icon: <YoutubeIcon className="w-10 h-10" />,
}]