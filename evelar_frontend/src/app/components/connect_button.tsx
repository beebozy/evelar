import { ConnectButton } from '@rainbow-me/rainbowkit';

export default function Button({}: { btnText: string}) {
    return (
      <div>
        <ConnectButton
          showBalance={false}
          accountStatus="address"
          chainStatus="icon"
          label="Connect Wallet"
     
  
        />
      </div>
    );
  }