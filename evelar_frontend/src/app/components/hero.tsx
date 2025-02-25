
import Link from "next/link";
import NavBar from "./nav";

interface HeroSectionProps {
  title: string;
  subtitle: string;
  description: string;
  showButton?: boolean;

//   children: React.ReactNode;
}

export default function HeroSection({ title, subtitle, description, showButton = false }: HeroSectionProps) {
  return (
    <section className="relative flex flex-col items-center justify-center h-screen bg-[#07091E] opacity-60 text-white bg-hero">
      {/* <div className="absolute inset-0 bg-grid-white/[0.1]" aria-hidden="true"></div> */}
      <NavBar />
      <div className="text-center max-w-3xl px-4">
        <h2 className="text-4xl md:text-6xl font-bold">{title}</h2>
        <h3 className="text-3xl md:text-5xl font-bold mt-2">{subtitle}</h3>
        <p className="mt-4 text-gray-400 text-lg">{description}</p>
      </div>
    {showButton ?  <Link href="/createvent" className="mt-4 border border-[#00e2ff] text-[#00e2ff] px-6 py-2 hover:bg-[#00e2ff]/10 rounded-3xl bg-transparent">Get Started</Link> : null}
    </section>
  );
}