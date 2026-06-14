import {
    Card,
    CardContent,
} from "@workspace/ui/components/card"
import { Spinner } from "@workspace/ui/components/spinner"
import { Button } from "@workspace/ui/components/button"
import { Dot } from "lucide-react"
import { useMutation } from '@tanstack/react-query'
import ShutdownDialog from './-ShutDownDialog'
import { useState } from "react"
import { useConfigurationStore } from '@/stores'
import { sendShutDownCommand, sendRebootCommand } from '@/apis'
import { toast } from "sonner"
interface OsProps {
    data: boolean,
    isLoading: boolean,
    className?: string
}
export default function OsCard(props: OsProps) {
    const { isLoading, data, className } = props
    const [dialogOpen, setDialogOpen] = useState(false)
    const [mode, setMode] = useState<"shutdown" | "reboot">("shutdown")

    const configData = useConfigurationStore((state) => state.config)
    const { protocol } = window.location
    const baseUrl = protocol + "//" + configData?.host + ":" + configData?.port
    const mutationShutdown = useMutation({
        mutationKey: ["setCommand"],
        mutationFn: async (params: { key: string, immediate: boolean }) => {
            if (!baseUrl) throw new Error("No URL provided")
            const response = await sendShutDownCommand({ config: { baseUrl }, data: params })
            return response
        },
    })
    const mutationReboot = useMutation({
        mutationKey: ["setCommand"],
        mutationFn: async (params: { key: string, immediate: boolean }) => {
            if (!baseUrl) throw new Error("No URL provided")
            const response = await sendRebootCommand({ config: { baseUrl }, data: params })
            return response
        },
    })
    const openDialog = (mode: 'reboot' | 'shutdown') => {
        setDialogOpen(true)
        setMode(mode)
    }
    const handleConfirmShutdown = (values: { password: string, immediate: boolean }) => {
        setDialogOpen(false)
        const params = {
            key: values.password,
            immediate: values.immediate
        }
        if (mode === 'reboot') {
            mutationReboot.mutateAsync(params).then(res => {
                console.log("res", res)
                if (!res.success) {
                    toast.error(res.msg || '重启命令失败')
                }
            }).catch(err => {
                console.log('errr;', err)
            })
        }
        else {
            mutationShutdown.mutateAsync(params).then(res => {
                console.log("res", res)
                if (!res.success) {
                    toast.error(res.msg || '关机失败')
                }
            }).catch(err => {
                console.log('errr;', err)
            })
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
