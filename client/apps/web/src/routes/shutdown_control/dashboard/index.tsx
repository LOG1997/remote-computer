import { createFileRoute } from "@tanstack/react-router"
import OsCard from "./components/OsCard/-index"
import CpuCard from "./components/CpuCard/-index"
import MemoryCard from "./components/MemoryCard/-index"
import StatusCard from "./components/StatusCard/-index"
import { useEffect } from "react"
import { LeftHeader } from "./Header/-left"
import { useWebSocket } from "@/components/WebsocketProvider"
import { useState } from "react"

export const Route = createFileRoute("/shutdown_control/dashboard/")({
    component: Dashboard,
    loader: async () => {
        return {
            meta: {
                back: "/ws_config",
                backName: "去配置",
            },
            header: {
                left: <LeftHeader />,
            },
        }
    },
})
function Dashboard() {
    const { sendMessage, subscribe, readyState } = useWebSocket()
    const [isLoading, setIsLoading] = useState(true)
    const [statusData, setStatusData] = useState(false)
    const [deviceData, setDeviceData] = useState({
        os: undefined,
        cpu: undefined,
        memory: undefined,
    })

    useEffect(() => {
        // 订阅 'get_system_info' 类型的消息
        const unsubscribe = subscribe((data: any) => {
            setIsLoading(false)
            setStatusData(true)
            console.log("收到消息", data)
            setDeviceData(data)
        }, "GetSystemInfo")

        return unsubscribe // 组件卸载时自动取消订阅
    }, [subscribe])

    useEffect(() => {
        sendMessage({
            topic: "GetSystemInfo",
            token: "1231212",
            date_time: new Date().getTime(),
            command: {
                command_type: "get_system_info",
            },
        })
        let timer = setInterval(() => {
            sendMessage({
                topic: "GetSystemInfo",
                token: "1231212",
                date_time: new Date().getTime(),
                command: {
                    command_type: "get_system_info",
                },
            })
        }, 10000)
        return () => {
            if (timer) {
                clearInterval(timer)
            }
        }
    }, [])

    return (
        <div className="flex flex-col gap-8">
            <StatusCard
                data={readyState === WebSocket.OPEN && statusData}
                isLoading={isLoading}
                className="h-18"
            />
            <OsCard
                data={readyState === WebSocket.OPEN ? deviceData.os : undefined}
                isLoading={isLoading}
                className="h-32"
            />
            <CpuCard
                data={readyState === WebSocket.OPEN ? deviceData?.cpu : undefined}
                isLoading={isLoading}
                className="h-42"
            />
            <MemoryCard
                data={readyState === WebSocket.OPEN ? deviceData?.memory : undefined}
                isLoading={isLoading}
                className="h-40"
            />
        </div>
    )
}
