// import Image from "next/image";


import CreateEvent  from "./components/CreateEvent";
export default function Home() {
  return (
    // <div>
    <div>
      <Hero title={"One Event At A Time"} subtitle={"Onchain Event Creation With Evelar"} description={"Establishing a comprehensive digital infrastructure. Our digital platform offers services ranging from digitizing pathology slides to AI"}></Hero>
      {/* <div> </div> */}
      <EventSection></EventSection>
      <InfoSection></InfoSection>
      <Other></Other>
      <TopEvent></TopEvent>
      <Bottomnav></Bottomnav>
      
    </div>

  );
}
