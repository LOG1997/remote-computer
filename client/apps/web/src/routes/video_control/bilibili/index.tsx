import { createFileRoute } from '@tanstack/react-router'
import { Button } from '@workspace/ui/components/button'
import { useMqtt } from '@/components/mqtt/MqttContext'
import { Field } from "@workspace/ui/components/field"
import { Input } from "@workspace/ui/components/input"
import { useState } from "react"
import { DirectionPart } from './parts/_Direction'
import { OtherActionGroup } from './parts/_OtherActionGroup'
import { CenterHeader } from '../Header/_center'
import { RightHeader } from '../Header/_right'

export const Route = createFileRoute('/video_control/bilibili/')({
    loader: async () => {
        return {
            header: {
                center: <CenterHeader />,
                right: <RightHeader />,
            },
        }
    },
    component: function RouteComponent() {
        const [searchValue, setSearchValue] = useState('')
        const mqttClient = useMqtt()
        const handleSearch = () => {
            mqttClient.publish('tv-web/log1997/receive', {
                action: 'bilibili',
                data: 'search',
                payload: searchValue
            })
        }
        const handleNavigate = (direction: 'back' | 'forward') => {
            mqttClient.publish('tv-web/log1997/receive', {
                action: 'navigate',
                data: direction
            })
        }

        return <div>
            <DirectionPart />
            <OtherActionGroup />
            <Button onClick={() => handleNavigate('back')}>
                后退
            </Button>
            <Button onClick={() => handleNavigate('forward')}>
                前进
            </Button>
        </div>
    },
})


