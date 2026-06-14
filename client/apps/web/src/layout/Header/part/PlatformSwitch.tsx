import React from 'react'
import {
    NavigationMenu,
    NavigationMenuContent,
    NavigationMenuItem,
    NavigationMenuLink,
    NavigationMenuList,
    NavigationMenuTrigger,
} from "@/components/ui/navigation-menu"
import { usePlatform } from '@/stores'
import { PlatformList } from '@/constant/platform'
export function PlatformSwitch() {
    const configData = usePlatform((state) => state.platformInfo)
    const setConfig = usePlatform((state) => state.setConfig)

    const handleSelect = (selectedItem: any) => {
        const targetPlatform = PlatformList.find((item) => item.name === selectedItem.name)
        setConfig({
            current: targetPlatform,
        })
    }
    return (
        <div>
            <NavigationMenu>
                <NavigationMenuList>
                    <NavigationMenuItem>
                        <NavigationMenuTrigger>
                            {
                                configData && configData.current ? configData.current?.name : 'none'
                            }
                        </NavigationMenuTrigger>
                        <NavigationMenuContent>
                            {
                                PlatformList.map((item) => {
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

        </div >
    )
}
