(ns battlesnake.handler
  (:require [compojure.core :as w]
            [compojure.route :as route]
            [ring.middleware.defaults :refer [wrap-defaults site-defaults]]
            [ring.middleware.json :refer [wrap-json-response wrap-json-body]]))

(w/defroutes app-routes
  (w/GET "/" [] "Hello World")
  (route/not-found "Not Found"))

(def app
  (-> app-routes
      (wrap-defaults site-defaults)
      (wrap-json-response)
      (wrap-json-body {:keywords? true})))
