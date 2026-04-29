import { StrictMode } from "react"
import ReactDOM from "react-dom/client"
import { MqttProvider } from '@/components/mqtt/MqttContext'
import { RouterProvider, createRouter, createHashHistory } from '@tanstack/react-router'

import "@workspace/ui/globals.css"
import { routeTree } from './routeTree.gen'
const hashHistory = createHashHistory()

const router = createRouter({
    routeTree,
    history: hashHistory,
    defaultPreload: 'intent',
    scrollRestoration: true,
})

declare module '@tanstack/react-router' {
    interface Register {
        router: typeof router
    }
}
const rootElement = document.getElementById('app')!

if (!rootElement.innerHTML) {
    const root = ReactDOM.createRoot(rootElement)
    root.render(
        <StrictMode>
            <MqttProvider>
                <RouterProvider router={router} />
            </MqttProvider>
        </StrictMode>
    )


}
