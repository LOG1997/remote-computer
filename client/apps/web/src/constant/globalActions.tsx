import { RefreshCw, CircleX, ChevronRight, ChevronLeft } from 'lucide-react'
export const globalActions = [
    {
        name: 'refresh',
        value: 'refresh',
        icon: <RefreshCw />,
    },
    {
        name: 'closeTab',
        value: 'closeTab',
        icon: <CircleX />,
    },
    {
        name: 'nextTab',
        value: 'nextTab',
        icon: <ChevronRight />,
    },
    {
        name: 'previousTab',
        value: 'previousTab',
        icon: <ChevronLeft />,
    },
    {
        name: 'back',
        value: 'back',
        icon: <ChevronLeft />
    },
    {
        name: 'forward',
        value: 'forward',
        icon: <ChevronRight />
    }
]