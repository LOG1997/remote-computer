import { useMqtt } from '@/components/mqtt/MqttContext'
import { AlertTriangleIcon } from "lucide-react"
import {
    Alert,
    AlertDescription,
    AlertAction,
    AlertTitle,
} from "@workspace/ui/components/alert"
import { Button } from "@workspace/ui/components/button"
import { useNavigate } from "@tanstack/react-router"

export function MqttStatus() {
    const { isConnected } = useMqtt();
    const navigate = useNavigate();

    const gotoMqttConfig = () => {
        navigate({ to: '/mqtt_config' })
    }
    return (
        <div className="text-xs flex justify-center">
            {isConnected === false && <Alert className="max-w-md border-amber-200 bg-amber-50 text-amber-900 dark:border-amber-900 dark:bg-amber-950 dark:text-amber-50">
                <AlertTriangleIcon />
                <AlertTitle>there is no mqtt connection</AlertTitle>
                <AlertDescription>
                    Please check your mqtt server;or check your mqtt client config
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
