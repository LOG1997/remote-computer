import {
    NavigationMenu,
    NavigationMenuContent,
    NavigationMenuItem,
    NavigationMenuLink,
    NavigationMenuList,
    NavigationMenuTrigger,
} from "@workspace/ui/components/navigation-menu"
import { globalActions } from '@/constant/globalActions'
import { useWebSocket } from "@/components/ws/WebsocketProvider"
export function RightHeader() {
    const { sendMessage, subscribe, readyState } = useWebSocket()
    const handleSelect = (item: any) => {
        sendMessage({
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
