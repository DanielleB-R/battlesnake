(ns battlesnake.geometry-test
  (:require [clojure.test :refer :all]
            [battlesnake.geometry :refer :all]))

(deftest test-direction-adjacent
  (let [origin (coord 0 0)]

    (testing "left of origin"
      (is (= (direction-adjacent origin (coord -1 0)) :left)))

    (testing "right of origin"
      (is (= (direction-adjacent origin (coord 1 0)) :right)))

    (testing "above origin"
      (is (= (direction-adjacent origin (coord 0 1)) :up)))

    (testing "below origin"
      (is (= (direction-adjacent origin (coord 0 -1)) :down)))

    (testing "elsewhere"
      (is (nil? (direction-adjacent origin (coord 1 1))))
      (is (nil? (direction-adjacent origin (coord -1 -1))))
      (is (nil? (direction-adjacent origin (coord 2 3)))))))
