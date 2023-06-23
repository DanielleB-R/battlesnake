(ns battlesnake.handler-test
  (:require [clojure.test :refer :all]
            [ring.mock.request :as mock]
            [battlesnake.handler :refer [app snake-info]]
            [cheshire.core :as json]))

(deftest test-app
  (testing "main route"
    (let [response (app (mock/request :get "/"))]
      (is (= (:status response) 200))
      (is (= (json/parse-string (:body response) true) snake-info))))

  (testing "not-found route"
    (let [response (app (mock/request :get "/invalid"))]
      (is (= (:status response) 404)))))
