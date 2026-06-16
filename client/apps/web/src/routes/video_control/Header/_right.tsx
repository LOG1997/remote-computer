import {
    NavigationMenu,
    NavigationMenuContent,
    NavigationMenuItem,
    NavigationMenuLink,
    NavigationMenuList,
    NavigationMenuTrigger,
} from "@workspace/ui/components/navigation-menu"
import { globalActions } from '@/constant/globalActions'
import { useMqtt } from '@/components/mqtt/MqttContext'
export function RightHeader() {
    const mqttClient = useMqtt()
    const handleSelect = (item: any) => {
        mqttClient.publish('tv-web/log1997/receive', {
            action: 'global',
            data: item.value,
        })
    }
    return (
        <div>
            <NavigationMenu>
                <NavigationMenuList>
                    <NavigationMenuItem>
                        <NavigationMenuTrigger className="[&_svg]:hidden">
                            <span>...</span>
                        </NavigationMenuTrigger>
                        <NavigationMenuContent>
                            {
                                globalActions.map((item) => {
                                    return (
                                        <NavigationMenuLink key={item.name} onClick={() => {
                                            handleSelect(item)
                                        }}>
                                            {item.name}
                                        </NavigationMenuLink>
                                    )
                                })

                            }
                        </NavigationMenuContent>
                    </NavigationMenuItem>
                </NavigationMenuList>
            </NavigationMenu>
        </div>
    )
}
