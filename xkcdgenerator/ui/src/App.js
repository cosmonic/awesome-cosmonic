import React, { useState, useEffect } from 'react';
import {
  ChakraProvider,
  Box,
  Button,
  VStack,
  Flex,
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
            pos="absolute"
            bottom="0"
            right="0"
            p={2}
            w="18rem"
          />
        </a>
        <Flex align="center" justify="center" pos="absolute" direction="column" bottom="20" left="5">
          <Text as="h3" fontSize="xl">Keep going!</Text>
          <Button as="a" href="https://new.cosmonic.app/?yaml=https://raw.githubusercontent.com/cosmonic/awesome-cosmonic/main/xkcdgenerator/xkcd-generator-cosmonic.wadm.yaml" target="_blank" rel="noreferrer" margin="4px" width="100%" textAlign="left" textColor="#685BC7">
            Step 1: Deploy on Cosmonic
          </Button>
          <Button as="a" href="https://new.cosmonic.app/?yaml=https://raw.githubusercontent.com/cosmonic/awesome-cosmonic/main/xkcdgenerator/xkcd-generator-stargate.wadm.yaml" target="_blank" rel="noreferrer" margin="4px" width="100%" textAlign="left" textColor="#685BC7">
            Step 2. Distribute to your Edge
          </Button>
          <Button as="a" href="https://cosmonic.com/docs/getting-started/makeityourown" target="_blank" rel="noreferrer" margin="4px" width="100%" textAlign="left" textColor="#685BC7">
            Step 3. Make it Your Own
          </Button>
        </Flex>
      </Box>
    </ChakraProvider >
  );
}

export default App;
