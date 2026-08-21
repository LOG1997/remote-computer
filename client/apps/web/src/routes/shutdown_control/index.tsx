import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { useEffect } from 'react'
import { useWsConfig } from '@/stores'
export const Route = createFileRoute('/shutdown_control/')({
    component: RouteComponent,
})

function RouteComponent() {
    const navigate = useNavigate()
    const configData = useWsConfig((state) => state.wsConfig)
    useEffect(() => {
        if (!configData) {
            navigate({ to: '/ws_config' })
        }
        else {
            navigate({ to: '/shutdown_control/dashboard' })
        }
    }, [navigate, configData])
    return <div>Hello "/shutdown_control/"!</div>
}
