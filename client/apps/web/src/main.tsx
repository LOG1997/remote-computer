import { StrictMode } from "react"
import ReactDOM from "react-dom/client"
import { MqttProvider } from '@/components/mqtt/MqttContext'
import { WebSocketProvider } from "@/components/WebsocketProvider"
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
            <WebSocketProvider url="ws://127.0.0.1:52011/user" reconnectInterval={3000}>
                <RouterProvider router={router} />
        </WebSocketProvider>
        </MqttProvider>
        </StrictMode>
    )


}
