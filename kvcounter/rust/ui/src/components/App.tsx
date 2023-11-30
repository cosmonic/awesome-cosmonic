import { Fireworks, FireworksHandlers } from '@fireworks-js/react'
import { useState, useEffect, useRef, FormEvent } from 'react'
import api from '../services/ApiService'
import { ReactComponent as CosmonicLogo } from '../assets/cosmonic-logo.svg'
import { ReactComponent as GithubLogo } from '../assets/github-logo.svg'
import { useScreenSize } from '../hooks/useScreenSize'

function App() {
  const [bucket, setBucket] = useState('')
  const [count, setCount] = useState(0)
  const dimensions = useScreenSize()
  const fireworks = useRef<FireworksHandlers>(null)

  const updateCount = async (key?: string) => {
    try {
      const response = await api.increment(key)
      setCount(response.counter)
      return response.counter
    } catch (err) {
      console.log(err)
    }
  }

  const launch = (total: number) => {
    fireworks.current?.launch(total > 50 ? 50 : total)
  }

  useEffect(() => {
    fireworks.current?.updateSize(dimensions)
  }, [dimensions])

  const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    let workingTotal = count
    const el = e.currentTarget.elements.namedItem('bucket') as HTMLInputElement
    const newBucket = el?.value ?? ''
    if (newBucket !== bucket) {
      workingTotal = 0
    }
    updateCount(newBucket).then((newCount) => {
      setBucket(newBucket)
      launch(newCount - workingTotal)
    })
  }

  useEffect(() => {
    updateCount().then((newCount) => launch(newCount))
  }, [])

  return (
    <div className="h-full grid grid-rows-[1fr,auto]">
      <div className="flex flex-col gap-2 h-full items-center justify-center">
        <form className="flex flex-wrap gap-2" onSubmit={handleSubmit}>
          <input
            id="bucket"
            name="bucket"
            placeholder="Enter a bucket name"
            className="mx-auto px-2 py-1.5 text-center w-56 max-w-full text-cosmonicGray rounded-md border border-cosmonicPurple-light"
          />
          <button
            type="submit"
            className="bg-cosmonicPurple-light w-56 max-w-full rounded-md hover:bg-cosmonicPurple-dark text-white font-bold py-2 px-4 my-auto mx-auto"
          >
            Increment
          </button>
        </form>
        <h2 className="text-7xl mt-5 mx-auto font-bolder text-cosmonicPurple-light">
          {count}
        </h2>
      </div>
      <div className="flex justify-between w-screen p-4">
        <div className="flex items-center">
          <GithubLogo className="h-12 mr-4" />
          <a
            className="underline text-cosmonicPurple-dark"
            href="https://github.com/cosmonic/awesome-cosmonic/tree/main/kvcounter"
          >
            {' '}
            Check out the source code!{' '}
          </a>
        </div>
        <CosmonicLogo className="h-12" />
      </div>
      <Fireworks
        ref={fireworks}
        autostart={false}
        className="absolute inset-0 h-full w-full -z-10"
      />
    </div>
  )
}

export default App
