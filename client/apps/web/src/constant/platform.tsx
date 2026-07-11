import { BilibiliIcon } from '@/components/icons/Bilibili'
import { YoutubeIcon,DouyinIcon } from '@/components/icons'
export const PlatformList = [{
    name: 'bilibili',
    value: 'bilibili',
    path: '/video_control/bilibili',
    url: 'https://www.bilibili.com/',
    description: '哔哩哔哩',
    icon: <BilibiliIcon className="w-10 h-10 text-pink-400" />,
},
{
    name: '抖音',
    value: 'douyin',
    path: '/video_control/douyin',
    url: 'https://www.douyin.com/',
    description: '抖音',
    icon: <DouyinIcon className="w-10 h-10" />,
},
  {
    name: 'youtube',
    value: 'youtube',
    path: '/video_control/youtube',
    url: 'https://www.youtube.com/',
    description: 'YouTube',
    icon: <YoutubeIcon className="w-10 h-10" />,
}]
