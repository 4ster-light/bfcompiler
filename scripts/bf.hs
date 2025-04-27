module Main where

import Control.Exception (Exception, catch, throwIO)
import Control.Monad (when)
import Data.Array (Array, array, (!), (//))
import Data.Char (chr, ord)
import Data.IntMap.Strict (IntMap)
import Data.IntMap.Strict qualified as IntMap
import System.Environment (getArgs)
import System.IO (hFlush, stdout)

data BrainfuckError
  = MemoryOutOfBounds
  | UnmatchedBracket
  deriving (Show)

instance Exception BrainfuckError

maxProgSize :: Int
maxProgSize = 30000

checkBounds :: Int -> Array Int Int -> IO ()
checkBounds ptr mem = when (ptr < 0 || ptr >= maxProgSize) $ throwIO MemoryOutOfBounds

findMatchingBrackets :: String -> Either BrainfuckError (IntMap Int)
findMatchingBrackets code = go 0 [] IntMap.empty
  where
    go i stack brackets
      | i >= length code = if null stack then Right brackets else Left UnmatchedBracket
      | otherwise = case code !! i of
          '[' -> go (i + 1) (i : stack) brackets
          ']' -> case stack of
            openPos : rest ->
              let brackets' = IntMap.insert openPos i $ IntMap.insert i openPos brackets
               in go (i + 1) rest brackets'
            [] -> Left UnmatchedBracket
          _ -> go (i + 1) stack brackets

interpretBF :: String -> IntMap Int -> IO ()
interpretBF code brackets = do
  let memory = array (0, maxProgSize - 1) [(i, 0) | i <- [0 .. maxProgSize - 1]]
  loop 0 0 memory
  where
    loop ptr codePtr mem
      | codePtr >= length code = pure ()
      | otherwise = do
          checkBounds ptr mem
          case code !! codePtr of
            '+' -> loop ptr (codePtr + 1) $ mem // [(ptr, (mem ! ptr + 1) `mod` 256)]
            '-' -> loop ptr (codePtr + 1) $ mem // [(ptr, (mem ! ptr - 1) `mod` 256)]
            '<' -> loop (max 0 $ ptr - 1) (codePtr + 1) mem
            '>' -> loop (ptr + 1) (codePtr + 1) mem
            ',' -> do
              c <- getChar
              loop ptr (codePtr + 1) $ mem // [(ptr, ord c)]
            '.' -> do
              putChar $ chr $ mem ! ptr
              hFlush stdout
              loop ptr (codePtr + 1) mem
            '[' ->
              loop
                ptr
                (if mem ! ptr == 0 then brackets IntMap.! codePtr + 1 else codePtr + 1)
                mem
            ']' ->
              loop
                ptr
                (if mem ! ptr /= 0 then brackets IntMap.! codePtr else codePtr + 1)
                mem
            _ -> loop ptr (codePtr + 1) mem

main :: IO ()
main = do
  args <- getArgs
  case args of
    [filename] -> do
      code <- readFile filename
      case findMatchingBrackets code of
        Left err -> putStrLn $ "Error: " ++ show err
        Right brackets ->
          interpretBF code brackets `catch` \e ->
            putStrLn $ "Error: " ++ show (e :: BrainfuckError)
    _ -> putStrLn "Usage: runhaskell bf.hs <filename>"
