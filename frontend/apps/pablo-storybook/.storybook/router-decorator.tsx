import { action } from '@storybook/addon-actions';
import { RouterContext } from 'next/dist/shared/lib/router-context';
import { ReactNode } from 'react';

const getRouter = () => {
    return {
        locale: 'en',
        route: '/',
        pathname: '/',
        query: {},
        asPath: '/',
        push(...args: unknown[]) {
            action('nextRouter.push')(...args);
            return Promise.resolve(true);
        },
        replace(...args: unknown[]) {
            action('nextRouter.replace')(...args);
            return Promise.resolve(true);
        },
        reload(...args: unknown[]) {
            action('nextRouter.reload')(...args);
        },
        back(...args: unknown[]) {
            action('nextRouter.back')(...args);
        },
        prefetch(...args: unknown[]) {
            action('nextRouter.prefetch')(...args);
            return Promise.resolve();
        },
        beforePopState(...args: unknown[]) {
            action('nextRouter.beforePopState')(...args);
        },
        events: {
            on(...args: unknown[]) {
                action('nextRouter.events.on')(...args);
            },
            off(...args: unknown[]) {
                action('nextRouter.events.off')(...args);
            },
            emit(...args: unknown[]) {
                action('nextRouter.events.emit')(...args);
            },
        },
        isFallback: false,
        basePath: '',
        isLocaleDomain: false,
        isReady: true,
        isPreview: false,
    };
};

export const StorybookRouterProvider = ({ children }: { children: ReactNode }) => (
    <RouterContext.Provider value={getRouter()}>{children}</RouterContext.Provider>
);
