/**
 * 这是总的入口，目前包含功能
 * 1. 远程关机（包含电脑信息）
 * 2. 遥控器（遥控B站等网页端）
 */
import { createFileRoute } from '@tanstack/react-router'
import { useNavigate } from '@tanstack/react-router'
import { Button } from "@workspace/ui/components/button"

export const Route = createFileRoute('/home/')({
    component: RouteComponent,
})

function RouteComponent() {
    const navigate = useNavigate()
    const onClick = (targetPath: string) => {
        navigate({ to: targetPath })
    }
    return <div>
        <div>
            <Button onClick={() => onClick('/shutdown_control')}>远程关机</Button>
        </div>
        <div>
            <Button onClick={() => onClick('/video_control')}>遥控器</Button>
        </div>
    </div>
}
