import { router } from "@inertiajs/core";
import { Deferred, usePage } from "@inertiajs/react";

type User = {
    id: number;
    name: string;
}

type PageProps = {
    users: User[];
}

export default function BuiltInDeferred() {
    function handleTriggerPartialReload() {
        // basically, a reload. Could even be a form submitting to a route that returns `redirect()->back()` 
        router.post("pingpong");
    }

    return (
        <main className="main-container">
            <h1 className="mb-12 p-20 bg-amber-100">Listing users with <span className="underline">built-in</span> Deferred component</h1>

            <Deferred data="users" fallback={<DeferredUsersSkeleton />}>
                <DeferredUsers/>
            </Deferred>
            
            <button
                onClick={handleTriggerPartialReload}
                className="mt-8 btn"
            >
                Trigger a partial reload
            </button>
        </main>
    )
}

export function DeferredUsers() {
    const props = usePage<PageProps>().props;
    
    return (
      <div className="flex flex-col gap-1">
        {/* if it was props.users?.map instead, it wouldn't throw the error and the fallback would be shown again */}
        { props.users.map(UserCard) }
      </div>
    )
}

function UserCard(user: User) {
    return (
        <span
            key={`user-${user.id}`}
            className="px-4 py-2 bg-blue-100 font-medium text-slate-700"
        >
            {user.name}
        </span>
    )
}

function DeferredUsersSkeleton() {
    return (
        <div className="animate-pulse rounded-lg bg-blue-100 w-full h-36" />
    )
}