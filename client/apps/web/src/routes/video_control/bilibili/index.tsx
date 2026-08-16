import { createFileRoute } from '@tanstack/react-router'
import { DirectionPart } from './parts/-Direction'
import { OtherActionGroup } from './parts/-OtherActionGroup'
import { CenterHeader } from '../Header/-center'
import { RightHeader } from '../Header/-right'
import { VolumeAction } from './parts/-VolumeAction'
import { VideoOption } from './parts/-VideoOption'

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
        return <div>
            <VideoOption />
            <DirectionPart />
            <OtherActionGroup />
            <VolumeAction />
        </div>
    },
})


