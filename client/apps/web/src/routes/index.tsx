import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { useEffect } from 'react'

export const Route = createFileRoute('/')({
    component: RouteComponent,
})

function RouteComponent() {

    const navigate = useNavigate()
    useEffect(() => {
        navigate({ to: '/home' })
    }, [navigate])

    return <div>Redirecting to dashboard...</div>
}
