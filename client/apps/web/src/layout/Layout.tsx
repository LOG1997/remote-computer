import { ThemeProvider } from "@/components/theme-provider.tsx"
import { Toaster } from "@workspace/ui/components/sonner"
import { Outlet } from '@tanstack/react-router'
import Header from './Header';
import { toast } from "sonner"
import { useWebSocket } from "@/components/ws/WebsocketProvider"
import { useEffect } from "react";
import { WsStatusHeader } from '@/components/ws/WsStatus'

export default function Layout() {
    const { subscribe } = useWebSocket()

    useEffect(() => {
        // 订阅 'get_system_info' 类型的消息
        const unsubscribe = subscribe((data: any) => {
            let errMsg = data.msg;
            toast.error(errMsg || "出错了")
        }, "Error")

        return unsubscribe // 组件卸载时自动取消订阅
    }, [subscribe])

    useEffect(() => {
        const unsubscribe = subscribe((data: any) => {
            console.log("收到消息suoyoyu", data)
        }, "*")

        return unsubscribe // 组件卸载时自动取消订阅
    }, [subscribe])
    return (
        <>
            <ThemeProvider>
                <Header />
                <main className="mt-8">
                    <WsStatusHeader />
                    <Outlet />
                </main>
                <Toaster />
            </ThemeProvider>
        </>
    )
}
