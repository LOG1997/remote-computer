import {
    Card,
    CardContent,
} from "@workspace/ui/components/card"
import { Spinner } from "@workspace/ui/components/spinner"
import { Button } from "@workspace/ui/components/button"
import { Dot } from "lucide-react"
import ShutdownDialog from './-ShutDownDialog'
import { useState } from "react"
import { useWebSocket } from "@/components/ws/WebsocketProvider"
import { useEffect } from 'react'
interface OsProps {
    data: boolean,
    isLoading: boolean,
    className?: string
}
export default function OsCard(props: OsProps) {
    const { isLoading, data, className } = props
    const [dialogOpen, setDialogOpen] = useState(false)
    const [mode, setMode] = useState<"shutdown" | "reboot">("shutdown")

    const { sendMessage, subscribe } = useWebSocket()

    useEffect(() => {
        // 订阅 'get_system_info' 类型的消息
        const unsubscribe = subscribe((data: any) => {
            console.log("收到关机消息", data)
        }, "SystemControl")

        return unsubscribe // 组件卸载时自动取消订阅
    }, [subscribe])
    const triggerUpdate = (mode: String, param: { password: String, immediate: boolean }) => {
        sendMessage({
            topic: "SystemControl",
            token: "1231212",
            date_time: new Date().getTime(),
            command: {
                command_type: mode,
                param
            },
        })
    }

    const openDialog = (mode: 'reboot' | 'shutdown') => {
        setDialogOpen(true)
        setMode(mode)
    }
    const handleConfirmShutdown = (values: { password: string, immediate: boolean }) => {
        setDialogOpen(false)
        let param = {
            password: values.password,
            immediate: values.immediate
        }
        if (mode === 'reboot') {
            triggerUpdate("reboot", param)
        }
        else {
            triggerUpdate("shutdown", param)
        }

    }
    return (
        <div className={className + " flex justify-center items-center min-h-18"}>
            {
                isLoading ?
                    <Card className="relative mx-auto w-full h-full max-w-sm pt-0">
                        <Spinner className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2" />
                    </Card> :
                    <Card className="relative mx-auto w-full max-w-sm pt-0">
                        <CardContent className="flex justify-between items-center pt-3">
                            <div id="os-system" className=" flex flex-col gap-3 justify-center">
                                {
                                    data ?
                                        <div className="flex gap-1 items-center">
                                            <Dot className="text-green-500" strokeWidth={8} />
                                            <span>在线</span>
                                        </div> :
                                        <div className="flex gap-1 items-center">
                                            <Dot className="text-red-500" strokeWidth={8} />
                                            <span>离线</span>
                                        </div>
                                }
                            </div>
                            <div id="shut-down" className="pl-12 flex gap-2">
                                <Button variant="destructive" disabled={!data} onClick={() => { openDialog('shutdown') }}>关机</Button>
                                <Button variant="outline" disabled={!data} onClick={() => openDialog('reboot')} >重启</Button>
                            </div>

                        </CardContent>
                    </Card>
            }
            <ShutdownDialog
                open={dialogOpen}
                setOpen={setDialogOpen}
                handleSubmit={handleConfirmShutdown}
                mode={mode}
            />
        </div >
    )
}
