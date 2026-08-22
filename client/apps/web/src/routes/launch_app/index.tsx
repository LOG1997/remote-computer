import { createFileRoute } from '@tanstack/react-router'
import { AppLaunchList } from '@/constant/app'
import {
    Item,
    ItemContent,
    ItemDescription,
    ItemGroup,
    ItemHeader,
    ItemTitle,
} from "@workspace/ui/components/item"
import { useWebSocket } from "@/components/ws/WebsocketProvider"
import { toast } from "sonner"

export const Route = createFileRoute('/launch_app/')({
    loader: async () => {
        return {

        }
    },
    component: RouteComponent
})

function RouteComponent() {
    const { sendMessage, readyState } = useWebSocket()
    const handleLaunchApp = (targetApp: any) => {
        if (readyState !== WebSocket.OPEN) {
            toast.error('请先连接服务器')
            return
        }
        console.log('启动app', targetApp)
        sendMessage({
            topic: "LaunchApp",
            token: "1231212",
            date_time: new Date().getTime(),
            command: {
                command_type: "launch",
                param: targetApp.value
            },
        })
    }
    return (
        <div className="px-2">
            <ItemGroup className="grid grid-cols-3 gap-4">
                {AppLaunchList.map((app) => (
                    <Item key={app.name} variant="outline" onClick={() => { handleLaunchApp(app) }} className=" transition-transform active:bg-primary/5 active:text-primary active:scale-[0.98]  cursor-pointer select-none">
                        <ItemHeader>
                            <div className="w-full">
                                {
                                    app.icon && (
                                        app.icon
                                    )
                                }
                            </div>
                        </ItemHeader>
                        <ItemContent>
                            <ItemTitle className='select-none'>{app.name}</ItemTitle>
                            <ItemDescription className='select-none text-sm'>{app.description}</ItemDescription>
                        </ItemContent>
                    </Item>
                ))}
            </ItemGroup>
        </div >)
}
