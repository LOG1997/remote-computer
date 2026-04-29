import { createFileRoute } from '@tanstack/react-router'
import { menus } from './-config'
import { useNavigate } from '@tanstack/react-router'
import { useFullscreen } from 'ahooks';
import { useRef, useEffect } from 'react'


export const Route = createFileRoute('/remote-control/')({
    component: RouteComponent,
})

function RouteComponent() {
    const navigate = useNavigate()
    // const ref = useRef<HTMLDivElement | null>(null)
    const rootRef = useRef(document.documentElement);

    const [isFullscreen, { enterFullscreen, exitFullscreen, toggleFullscreen }] = useFullscreen(rootRef);
    function goRoute(route: string) {
        console.log('goRoute', route)
        // TODO:全屏展示
        // if (!isFullscreen) {
        //     enterFullscreen()
        // }
        if (route) {
            navigate({ to: route })
        }

    }
    return <div className='flex gap-3 text-gray-500/60 text-sm'>
        {
            menus.map((menu) => (
                <div key={menu.route} onClick={() => goRoute(menu.route)} className='size-24 cursor-pointer bg-background/20 border-none shadow-lg drop-shadow-xs rounded-2xl flex flex-col py-3 gap-2 items-center'>
                    {menu.icon}
                    <p>{menu.name}</p>
                </div>
            ))
        }
    </div>
}
