
// import { CustomConnectButton } from "./connect_button";
import NavBar from "./nav";

export default function HeroSection() {
  return (
    <section className="relative flex flex-col items-center justify-center h-screen bg-gradient-to-br from-[#07091E] to-[#0A0A12] text-white">
      <div className="absolute inset-0 bg-grid-white/[0.1]" aria-hidden="true"></div>
      <NavBar />
      <div className="text-center max-w-3xl px-4">
        <h2 className="text-4xl md:text-6xl font-bold">One Event At A Time</h2>
        <h3 className="text-3xl md:text-5xl font-bold mt-2">Onchain Event Creation With Evelar</h3>
        <p className="mt-4 text-gray-400 text-lg">Establishing a comprehensive digital infrastructure. Our digital platform offers services ranging from digitizing pathology slides to AI.</p>
       
      </div>
    </section>
  );
}
