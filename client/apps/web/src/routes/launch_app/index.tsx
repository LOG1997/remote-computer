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
import { useMqtt } from '@/components/mqtt/MqttContext'

export const Route = createFileRoute('/launch_app/')({
    loader: async () => {
        return {

        }
    },
    component: RouteComponent
})

function RouteComponent() {
    const { publish } = useMqtt()
    const handleLaunchApp = (targetApp: any) => {
        console.log('启动app', targetApp)
        const mqtt_params = {
            app_name: targetApp.value,
        }
        publish(
            'tv-web/control/launch_app',
            JSON.stringify(mqtt_params)
        )
    }
    return (<div>
        <ItemGroup className="grid grid-cols-3 gap-4">
            {AppLaunchList.map((app) => (
                <Item key={app.name} variant="outline" onClick={() => { handleLaunchApp(app) }}>
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
                        <ItemTitle>{app.name}</ItemTitle>
                        <ItemDescription>{app.description}</ItemDescription>
                    </ItemContent>
                </Item>
            ))}
        </ItemGroup>
    </div >)
}
