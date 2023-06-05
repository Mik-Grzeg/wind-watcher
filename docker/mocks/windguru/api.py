import falcon, json, logging, datetime

logging.basicConfig()
logging.getLogger().setLevel(logging.DEBUG)

class CookiesSetter(object):
    cookies = dict(session='1687f46c2ea20bf45f5b1aeea8209541',
    deviceid='9a9936815593629d897e4dfb8fb25058',
    langc='en-')

    def on_get(self, req, resp):
        resp.set_cookie('session', self.cookies['session'], path='/', secure=True, domain='localhost', max_age=1800, expires=datetime.datetime.now() + datetime.timedelta(hours=3))
        resp.set_cookie('deviceid', self.cookies['deviceid'], path='/', secure=True, max_age=1800, expires=datetime.datetime.now() + datetime.timedelta(hours=3))
        resp.set_cookie('langc', self.cookies['langc'], path='/', secure=True, max_age=1800, expires=datetime.datetime.now() + datetime.timedelta(hours=3))

class WindguruResource:
    def __init__(self):
        with open('meta_spot.json', 'r') as f:
            self.response_body_meta = json.load(f)

        with open('forecast_spot.json', 'r') as f:
            self.response_body_forecast = json.load(f)

        with open('station_data.json', 'r') as f:
            self.response_body_station_data = json.load(f)

    def on_get(self, req, resp):
        for cookie_name, cookie_val in CookiesSetter.cookies.items():
            if req.get_cookie_values(cookie_name)[0] != cookie_val or req.get_header('Referer') != 'https://www.windguru.cz':
                resp.status = falcon.HTTP_403
                return
            
        resp.content_type = falcon.MEDIA_JSON

        method = req.params.get('q')

        if method == 'forecast_spot':
            logging.info("requested forecast modele data")
            if req.params.get('id_spot') == '36048':
                 resp.text = json.dumps(self.response_body_meta)
            else: 
                return falcon.HTTPNotFound
        elif method == 'forecast':
            logging.info("requested forecast data")
            if req.params.get('id_model') == '3' \
                    and req.params.get('rundef') == '2023040812x0x240x0x240-2023040800x243x384x255x384' \
                    and req.params.get('initstr') == '2023040812' \
                    and req.params.get('id_spot') == '36048' \
                    and req.params.get('WGCACHEABLE') == '21600' \
                    and req.params.get('cachefix') == '27.85x-15.35x0':
                resp.text = json.dumps(self.response_body_forecast)
        elif method == 'station_data':
            logging.info("requested station data")
            if req.params.get('id_station') == '2764':
                logging.debug(f"data: {self.response_body_station_data}")
                resp.text = json.dumps(self.response_body_station_data)
            else: 
                return falcon.HTTPNotFound
        else: 
            resp.status = falcon.HTTP_400

api = falcon.App()
api.add_route('/', CookiesSetter())
api.add_route('/int/iapi.php', WindguruResource())

