'use client';

import type React from 'react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { WagmiProvider } from 'wagmi';
import {  lightTheme, RainbowKitProvider } from '@rainbow-me/rainbowkit';

import { config } from '../wagmi';
import { ToastContainer } from 'react-toastify';

const queryClient = new QueryClient();
const customTheme = lightTheme({
  accentColor: '#00E2FF', // Change this to your desired color
  accentColorForeground: 'white',
  borderRadius: 'medium',
  fontStack: 'system',
  overlayBlur: 'small',
});

export function Providers({ children }: { children: React.ReactNode }) {
  return (
    <WagmiProvider config={config}>
      <QueryClientProvider client={queryClient}>
        <ToastContainer />
        <RainbowKitProvider
            theme={customTheme}
            >{children}</RainbowKitProvider>
      </QueryClientProvider>
    </WagmiProvider>
  );
}
