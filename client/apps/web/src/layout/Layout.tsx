import { ThemeProvider } from "@/components/theme-provider.tsx"
import { Toaster } from "@workspace/ui/components/sonner"
import { Outlet } from '@tanstack/react-router'
import Header from './Header';
import { MqttStatus } from './MqttStatus'

export default function Layout() {
    return (
        <>
            <ThemeProvider>
                <Header />
                <MqttStatus />
                <main className="mt-8">
                    <Outlet />
                </main>
                <Toaster />
            </ThemeProvider>
        </>
    )
}
