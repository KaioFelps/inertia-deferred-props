import { router } from '@inertiajs/core';
import { usePage } from '@inertiajs/react'
import { memo, ReactElement, useEffect, useMemo, useState } from 'react'

interface DeferredProps {
  children: ReactElement | number | string;
  fallback: ReactElement | number | string;
  data: string | string[];
}

const CustomDeferred = memo(({ children, data, fallback }: DeferredProps) => {
  if (!data) {
    throw new Error('`<Deferred>` requires a `data` prop')
  }

  const [loaded, setLoaded] = useState(false)
  const pageProps = usePage().props
  const keys = useMemo(() => Array.isArray(data) ? data : [data], [data]);

  useEffect(() => {
    const removeCallback = router.on("start", () => {
      // forces the fallback to avoid trying to read undefined prop
      setLoaded(false);
    });

    return () => {
      removeCallback();
    }
  }, [])

  useEffect(() => {
    setLoaded(keys.every((key) => pageProps[key] !== undefined))
  }, [pageProps, keys])

  return loaded ? children : fallback
});

CustomDeferred.displayName = 'CustomDeferred'

export default CustomDeferred