import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { PlatformList } from '@/constant/platform'
import {
    Item,
    ItemContent,
    ItemDescription,
    ItemGroup,
    ItemHeader,
    ItemTitle,
} from "@workspace/ui/components/item"
import { usePlatform } from '@/stores'
import { RightHeader } from './Header/-right'
import { useEffect } from 'react'

export const Route = createFileRoute('/video_control/')({
    loader: async () => {
        return {
            header: {
                right: <RightHeader />,
            }
        }
    },
    component: RouteComponent
})

function RouteComponent() {
    const navigate = useNavigate()
    const setPlatformConfig = usePlatform((state) => state.setConfig)
    const handleSelectPlatform = (platform: any) => {
        setPlatformConfig({
            current: platform,
        })
        navigate({ to: platform.path })
    }
    useEffect(() => {
        setPlatformConfig({
            current: null,
        })
    }, [])
    return (<div>
        <ItemGroup className="grid grid-cols-3 gap-4">
            {PlatformList.map((platform) => (
                <Item key={platform.name} variant="outline" onClick={() => { handleSelectPlatform(platform) }}>
                    <ItemHeader>
                        <div className="w-full">
                            {
                                platform.icon && (
                                    platform.icon
                                )
                            }
                        </div>
                    </ItemHeader>
                    <ItemContent>
                        <ItemTitle>{platform.name}</ItemTitle>
                        <ItemDescription>{platform.description}</ItemDescription>
                    </ItemContent>
                </Item>
            ))}
        </ItemGroup>
    </div >)
}
