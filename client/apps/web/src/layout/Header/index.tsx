// /home/log1997/r/shutdown/client/apps/web/src/layout/Header.tsx
import { House } from 'lucide-react';
import { useMatches } from '@tanstack/react-router';
import { useNavigate } from '@tanstack/react-router'
import { PlatformSwitch } from './part/PlatformSwitch'
import { GlobalAction } from './part/GlobalAction'
import { ChevronLeft } from 'lucide-react';
export default function Header() {
    // 示例：用于控制右侧按钮状态的 state
    const navigate = useNavigate()


    const matches = useMatches()
    const currentMatch = matches[matches.length - 1]
    const loaderData = currentMatch?.loaderData as any
    const gotoRoute = () => {
        navigate({ to: loaderData.meta.back })
    }
    return (
        <header className="flex items-center justify-between px-6 h-16 bg-white border-b border-gray-200 shadow-sm">
            {
                loaderData && loaderData.meta && loaderData.meta.back ?
                    <div className="shrink-0 flex items-center" onClick={gotoRoute}>
                        <ChevronLeft className="h-6 w-6 text-gray-600" />
                        <span className='text-xs'>{loaderData.meta.backName}</span>
                    </div> :
                    <div className="shrink-0 flex items-center" onClick={gotoRoute}>
                        <House className="h-6 w-6 text-gray-600" />
                    </div>
            }

            {/* 中间：切换平台 */}
            <div className="flex-1 text-center flex justify-center">
                <PlatformSwitch />
            </div>

            {/* 右边： 全局按钮*/}
            <div className="shrink-0">
                < GlobalAction />
            </div>
        </header>
    );
}