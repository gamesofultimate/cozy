import React from 'react';


type GradientMap = {
  width: number;
  height: number;
  radius: number;
  depth: number;
};

const Map: React.FC<GradientMap> = ({ width, height, radius, depth }) => {
  return (
    <svg id="map" height={height} width={width} viewBox={`0 0 ${width} ${height}`} xmlns="http://www.w3.org/2000/svg">
      <defs>
        <linearGradient 
          id="Y" 
          x1="0" 
          x2="0" 
          y1={`${Math.ceil((radius / height) * 15)}%`}
          y2={`${Math.floor(100 - (radius / height) * 15)}%`}
        >
          <stop offset="0%" stop-color="#0F0" />
          <stop offset="100%" stop-color="#000" />
        </linearGradient>
        <linearGradient 
          id="X" 
          x1={`${Math.ceil((radius / width) * 15)}%`} 
          x2={`${Math.floor(100 - (radius / width) * 15)}%`}
          y1="0" 
          y2="0"
        >
          <stop offset="0%" stop-color="#F00" />
          <stop offset="100%" stop-color="#000" />
        </linearGradient>
      </defs>

      <rect x="0" y="0" height={height} width={width} fill="#808080" />
      <g filter="blur(2px)">
        <rect x="0" y="0" height={height} width={width} fill="#000080" />
        <rect
            x="0"
            y="0"
            height={height}
            width={width}
            fill="url(#Y)"
            style={{ mixBlendMode: 'screen' }}
        />
        <rect
            x="0"
            y="0"
            height={height}
            width={width}
            fill="url(#X)"
        />
        <rect
            x={depth}
            y={depth}
            height={height - 2 * depth}
            width={width - 2 * depth}
            fill="#808080"
            rx={radius}
            ry={radius}
            filter={`blur(${depth}px)`}
        />
      </g>
    </svg>
  );
};

export type DisplacementOptions = {
  height: number;
  width: number;
  radius: number;
  depth: number;
  strength?: number;
  chromaticAberration?: number;
};

const Gradient: React.FC<DisplacementOptions> = ({
  height,
  width,
  radius,
  depth,
  strength = 100,
  chromaticAberration = 0,
}) => {
  return (
    <>
      {/*
      <Map width={width} height={height} radius={radius} depth={depth} />
      <svg id="aberration" height={height} width={width} viewBox={`0 0 ${width} ${height}`} xmlns="http://www.w3.org/2000/svg">
        <defs>
            <filter id="displace" color-interpolation-filters="sRGB">
                <feImage x="0" y="0" height={height} width={width} href="#map" result="displacementMap" />
                <feDisplacementMap
                    transform-origin="center"
                    in="SourceGraphic"
                    in2="displacementMap"
                    scale={strength + chromaticAberration * 2}
                    xChannelSelector="R"
                    yChannelSelector="G"
                />
                <feColorMatrix
                type="matrix"
                values="1 0 0 0 0
                        0 0 0 0 0
                        0 0 0 0 0
                        0 0 0 1 0"
                result="displacedR"
                        />
                <feDisplacementMap
                    in="SourceGraphic"
                    in2="displacementMap"
                    scale={strength + chromaticAberration}
                    xChannelSelector="R"
                    yChannelSelector="G"
                />
                <feColorMatrix
                type="matrix"
                values="0 0 0 0 0
                        0 1 0 0 0
                        0 0 0 0 0
                        0 0 0 1 0"
                result="displacedG"
                        />
                <feDisplacementMap
                        in="SourceGraphic"
                        in2="displacementMap"
                        scale={strength}
                        xChannelSelector="R"
                        yChannelSelector="G"
                    />
                    <feColorMatrix
                    type="matrix"
                    values="0 0 0 0 0
                            0 0 0 0 0
                            0 0 1 0 0
                            0 0 0 1 0"
                    result="displacedB"
                            />
                  <feBlend in="displacedR" in2="displacedG" mode="screen"/>
                  <feBlend in2="displacedB" mode="screen"/>
            </filter>
        </defs>
    </svg>
        */}
    <svg id="aberration" height={height} width={width} viewBox={`0 0 ${width} ${height}`} xmlns="http://www.w3.org/2000/svg">
    <filter id="liquid">
      <feTurbulence type="turbulence" baseFrequency="0.01 0.02" numOctaves="3" result="turbulence" seed="2">
        <animate attributeName="baseFrequency" dur="20s" values="0.01 0.02; 0.02 0.01; 0.01 0.02" repeatCount="indefinite" />
      </feTurbulence>
      <feDisplacementMap in2="turbulence" in="SourceGraphic" scale="30" xChannelSelector="R" yChannelSelector="G"/>
    </filter>
    </svg>
    </>
  );
};

export default Gradient;
