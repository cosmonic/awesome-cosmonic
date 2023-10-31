import React, { useState, useEffect } from 'react';
import {
  ChakraProvider,
  Box,
  Button,
  VStack,
  Grid,
  Text,
  theme,
  Image
} from '@chakra-ui/react';
import { ColorModeSwitcher } from './ColorModeSwitcher';

function App() {
  const [xkcdMetadata, setXkcdMetadata] = useState(null);
  const [xkcdLoading, setXkcdLoading] = useState(false);

  const fetchComic = async () => {
    setXkcdLoading(true);
    await fetch('/comic').then(response => response.json())
      .then(data => {
        setXkcdLoading(false);
        setXkcdMetadata(data);
      })
      .catch(error => {
        console.error('Error fetching comic:', error);
      });
  };

  useEffect(() => {
    fetchComic()
  }, []);

  return (
    <ChakraProvider theme={theme}>
      <Box textAlign="center" fontSize="xl">
        <Grid minH="100vh" p={3}>
          <ColorModeSwitcher justifySelf="flex-end" />
          <VStack height="90vh" spacing={8} align="center" justify="flex-start">
            <Button onClick={fetchComic} mb="4" textColor="#685BC7">
              Generate random XKCD comic
            </Button>
            {xkcdMetadata ? (
              <div>
                <Text fontSize="3xl" fontWeight="800" mb="8">{xkcdMetadata.title}</Text>
                <Image src={xkcdMetadata.img} alt={xkcdMetadata.title} />
              </div>
            ) : xkcdLoading ? <Text fontSize="3xl" fontWeight="800">Loading comic...</Text> : undefined}
          </VStack>
        </Grid>
        <a href="https://github.com/cosmonic/awesome-cosmonic/tree/main/xkcdgenerator" target="_blank" rel="noreferrer">
          <Image
            src="Cosmonic.Logo-Hrztl_Color.svg"  // Update with the correct path
            alt="Cosmonic Logo"
            style={{
              position: 'absolute',
              bottom: 0,
              right: 0,
              padding: '8px',
              width: '18rem'
            }}
          />
        </a>
        <div style={{
          position: 'absolute',
          bottom: 0,
          left: 0,
          display: 'flex',
          flexDirection: 'column',
        }}>
          <b><h1>Keep going!</h1></b>
          <a href="https://new.cosmonic.app/?yaml=https://raw.githubusercontent.com/cosmonic/awesome-cosmonic/main/xkcdgenerator/xkcd-generator-cosmonic.wadm.yaml" target="_blank" rel="noreferrer">
            <Button margin="4px" width="100%" textAlign="left" textColor="#685BC7">
              Step 1: Deploy on Cosmonic
            </Button>
          </a>
          <a href="https://new.cosmonic.app/?yaml=https://raw.githubusercontent.com/cosmonic/awesome-cosmonic/main/xkcdgenerator/xkcd-generator-stargate.wadm.yaml" target="_blank" rel="noreferrer">
            <Button margin="4px" width="100%" textAlign="left" textColor="#685BC7">
              Step 2. Distribute to your Edge
            </Button>
          </a>
          <a href="https://cosmonic.com/docs/getting-started/makeityourown" target="_blank" rel="noreferrer">
            <Button margin="4px" width="100%" textAlign="left" textColor="#685BC7">
              Step 3. Make it Your Own
            </Button>
          </a>
        </div>
      </Box>
    </ChakraProvider >
  );
}

export default App;
