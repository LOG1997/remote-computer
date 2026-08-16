import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { useEffect } from 'react'
import { useConfigurationStore } from '@/stores'
export const Route = createFileRoute('/shutdown_control/')({
    component: RouteComponent,
})

function RouteComponent() {
    const navigate = useNavigate()
    const configData = useConfigurationStore((state) => state.config)
    useEffect(() => {
        if (!configData) {
            navigate({ to: '/shutdown_control/config' })
        }
        else {
            navigate({ to: '/shutdown_control/dashboard' })
        }
    }, [navigate, configData])
    return <div>Hello "/shutdown_control/"!</div>
}
