import { ShutDownForm } from '@/components/form'
import { Toaster } from "@workspace/ui/components/sonner"
export function App() {
    return (
        <div className="flex min-h-svh justify-center">
            <div className='flex items-center w-full p-6'>
                <ShutDownForm />
            </div>
            <Toaster />
        </div >
    )
}
