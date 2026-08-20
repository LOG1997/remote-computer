import { StrictMode } from "react"
import ReactDOM from "react-dom/client"
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
    let ws_host = window.location.hostname;
    let ws_port = 52011
    root.render(
        <StrictMode>
            <WebSocketProvider url={`ws://${ws_host}:${ws_port}/user?token=121212`} reconnectInterval={3000}>
                <RouterProvider router={router} />
            </WebSocketProvider>
        </StrictMode >
    )


}
