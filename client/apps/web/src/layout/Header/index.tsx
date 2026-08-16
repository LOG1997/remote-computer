// /home/log1997/r/shutdown/client/apps/web/src/layout/Header.tsx
import { House } from 'lucide-react';
import { useMatches } from '@tanstack/react-router';
import { useNavigate } from '@tanstack/react-router'
export default function Header() {
    // 示例：用于控制右侧按钮状态的 state
    const navigate = useNavigate()
    const matches = useMatches()
    const currentMatch = matches[matches.length - 1]
    const loaderData = currentMatch?.loaderData as any
    const gotoHome = () => {
        navigate({ to: "/home" })
    }
    return (
        <header className="flex items-center justify-between px-6 h-16 bg-white border-b border-gray-200 shadow-sm">
            {
                loaderData && loaderData.header && loaderData.header.left ?
                    loaderData.header.left :
                    <div className="shrink-0 flex items-center" onClick={gotoHome}>
                        <House className="h-6 w-6 text-gray-600" />
                    </div>
            }

            {/* 中间：切换平台 */}
            {
                loaderData && loaderData.header && loaderData.header.center ?
                    loaderData.header.center :
                    <div className="flex-1 text-center flex justify-center" onClick={gotoHome}>
                        {/* <PlatformSwitch /> */}
                        Box Saka
                    </div>
            }

            {/* 右边： 全局按钮*/}
            {
                loaderData && loaderData.header && loaderData.header.right ?
                    loaderData.header.right : <div></div>

            }
            {/* <div className="shrink-0">
                <GlobalAction />
            </div> */}
        </header>
    );
}