import { createFileRoute } from '@tanstack/react-router'
import { Button } from '@workspace/ui/components/button'
import { useMqtt } from '@/components/mqtt/MqttContext'
import { DirectionPart } from './parts/_Direction'
import { OtherActionGroup } from './parts/_OtherActionGroup'
import { CenterHeader } from '../Header/_center'
import { RightHeader } from '../Header/_right'
import { VolumeAction } from './parts/_VolumeAction'

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
        const mqttClient = useMqtt()

        const handleNavigate = (direction: 'back' | 'forward') => {
            mqttClient.publish('tv-web/log1997/receive', {
                action: 'navigate',
                data: direction
            })
        }

        return <div>
            <DirectionPart />
            <OtherActionGroup />
            <VolumeAction />
            <Button onClick={() => handleNavigate('back')}>
                后退
            </Button>
            <Button onClick={() => handleNavigate('forward')}>
                前进
            </Button>
        </div>
    },
})


