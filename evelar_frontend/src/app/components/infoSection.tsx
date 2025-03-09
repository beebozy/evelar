/* eslint-disable @next/next/no-img-element */
import React from 'react'

export const InfoSection = () => {
    return (
        <div className='flex flex-col mt-20 bg-[#07091E] opacity-60 text-white '>
                  <h3 className="px-40 text-4xl font-bold bg-gradient-to-r from-[#FFAAF7] to-[#00E2FF] bg-clip-text text-transparent">
                  One Event At A Time <br></br> Onchain Event Creation with Evelar</h3>
            <div className="flex justify-between items-center">
                <div className="">
                    <div className="relative w-[600px] h-[400px]">
                        <img src="/glow.png" alt="image" className="absolute inset-0 " />
                        <img src="/glow1.png" alt="image" className="absolute inset-0" />
                    </div>
                </div>
                <p className=" w-[440px] text-gray-400 text-[#B7B7B7] mx-40">
                Evelar empowers individuals and organizations to seamlessly create and attend events of their choice. Our platform provides the flexibility to host events, participate in them, and issue certificates of participation in the form of NFTs. With no geographical restrictions, Evelar is accessible to users worldwide, ensuring a truly borderless event experience.</p>
            </div>
        </div>
    );
}
