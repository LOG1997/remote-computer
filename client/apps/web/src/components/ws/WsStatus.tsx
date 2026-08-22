import { useWebSocket } from "@/components/ws/WebsocketProvider"
import { AlertTriangleIcon } from "lucide-react"
import {
    Alert,
    AlertDescription,
    AlertAction,
    AlertTitle,
} from "@workspace/ui/components/alert"
import { Button } from "@workspace/ui/components/button"
import { useNavigate } from "@tanstack/react-router"

export function WsStatusHeader() {
    const { readyState } = useWebSocket()
    const navigate = useNavigate();

    const gotoMqttConfig = () => {
        navigate({ to: '/ws_config' })
    }
    return (
        <div className="text-xs flex justify-center">
            {readyState !== WebSocket.OPEN && <Alert className="max-w-md border-amber-200 bg-amber-50 text-amber-900 dark:border-amber-900 dark:bg-amber-950 dark:text-amber-50">
                <AlertTriangleIcon />
                <AlertTitle>there is no ws connection</AlertTitle>
                <AlertDescription>
                    Please check your ws server; or check your ws client config
                </AlertDescription>
                <AlertAction>
                    <Button size="xs" variant="default" onClick={gotoMqttConfig}>
                        Config
                    </Button>
                </AlertAction>
            </Alert>}
        </div>
    )
}