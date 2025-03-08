// import Image from "next/image";


import { Bottomnav } from "./components/bottomnav";
// import CreateEvent  from "./components/CreateEvent";
import { EventSection } from "./components/event_section";
import Hero from "./components/hero";
import { InfoSection } from "./components/infoSection";
import { Other } from "./components/other";
import { TopEvent } from "./components/topEvent";
export default function Home() {
  return (
    // <div>
    <div>
      <Hero title={"Bring Your Events Onchain with Evelar – Create, Host, and Discover!"} subtitle={""} description={"Evelar empowers creators and event enthusiasts to effortlessly organize and attend events, leveraging blockchain decentralization for a stress-free experience."}showButton={true}></Hero>
      {/* <div> </div> */}
      <EventSection></EventSection>
      <InfoSection></InfoSection>
      <Other></Other>
      <TopEvent></TopEvent>
      <Bottomnav></Bottomnav>
      
    </div>

  );
}
